class ContainerUse
  def initialize(@volume : Int32, @quantity = 0)
  end

  def incremented
    @quantity += 1
    self
  end

  def quantity
    @quantity
  end

  def reset_quantity
    @quantity = 0
  end

  def capacity
    @volume * @quantity
  end

  # def clone
  #   ContainerUse.new(@volume, @quantity)
  # end

  def as_tuple
    {@volume, @quantity}
  end
end

class ContainerCombo
  @containers : Array(ContainerUse)

  def initialize(container_volumes : Array(Int32))
    @containers = container_volumes.map { |vol| ContainerUse.new(vol) }
  end

  # def define_container_uses(container_uses : Array(ContainerUse))
  #   @containers = container_uses
  #   self
  # end

  def use_at(indexes : Array(Int32))
    @containers.map(&.reset_quantity)
    indexes.each { |index| incremented_at index }
    self
  end

  def incremented_at(index : Int32)
    @containers[index] = @containers[index].incremented
    self
  end

  # def any_over?(use_limit)
  #   @containers.any? { |container| container.quantity > use_limit }
  # end

  def total
    @containers.map(&.capacity).reduce(0) { |accum, capacity| accum + capacity }
  end

  def size
    @containers.size
  end

  # def clone
  #   new_combo = ContainerCombo.new([] of Int32)
  #   new_combo.define_container_uses(@containers.map(&.clone))
  #   new_combo
  # end

  def as_array
    @containers.map(&.as_tuple)
  end
end

def containers_parsed(filename)
  File.read(filename).split("\n").grep(/\d/).map(&.to_i)
end

# def combos(filename, volume, use_limit = nil)
#   container_combo = ContainerCombo.new(containers_parsed(filename))
#   solutions = combos_recursive(container_combo, volume, use_limit)
#   solutions.map(&.as_array).uniq
# end

# def combos_recursive(container_combo, volume, use_limit)
#   return [] of ContainerCombo if use_limit && container_combo.any_over?(use_limit)

#   total = container_combo.total
#   if total > volume
#     [] of ContainerCombo
#   elsif total == volume
#     [container_combo]
#   else
#     possibilities = (0..(container_combo.size - 1)).map do |index|
#       possibility = container_combo.clone
#       possibility.incremented_at(index)
#     end
#     possibilities.reject do |combo|
#       use_limit && combo.any_over?(use_limit)
#     end.map do |combo|
#       combos_recursive(combo, volume, use_limit).as(Array(ContainerCombo))
#     end.flatten
#   end
# end

# def how_many_minimal(combos : Array(Array(Tuple(Int32, Int32))))
#   containers_used = combos.map do |combo|
#     combo.map(&.last).reject { |quantity| quantity == 0 }.size
#   end
#   minimum_usage = containers_used.min
#   containers_used.select { |usage| usage == minimum_usage }.size
# end

# puts "example case:"
# puts "all combos with use limit 1:"
# result = combos("aoc_2015_17_input0.txt", 25, 1)
# p "There are #{result.map { |thing| p thing }.size} combos."
# p "There are #{how_many_minimal(result)} minimal solutions."

# puts "question 1 & 2:"
# puts "with use limit 1:"
# result = combos("aoc_2015_17_input1.txt", 150, 1)
# p "There are #{result.map { |thing| p thing }.size} combos."
# p "There are #{how_many_minimal(result)} minimal solutions."


# For performance and sᎥmplᎥcᎥty, limit possibilities to one use per container only
def more_performant_combos(filename, volume)
  container_combo = ContainerCombo.new(containers_parsed(filename))
  return container_combo.as_array.map(&.to_a) if volume <= 0

  container_count = container_combo.size
  indexes = 0.upto(container_count - 1).to_a
  (1..(container_count - 1)).flat_map do |usage_count|
    indexes.combinations(usage_count).select do |usages|
      container_combo.use_at(usages).total == volume
    end
  end
end

def how_many_minimal_from_indexes(indexes : Array(Array(Int32)))
  usages = indexes.map(&.size)
  min_usage = usages.min
  usages.select { |usage| usage == min_usage }.size
end

puts "Example case:"
p "Solutions: #{usage_indexes = more_performant_combos("aoc_2015_17_input0.txt", 25)}."
p "There are #{usage_indexes.size} solutions."
p "There are #{how_many_minimal_from_indexes(usage_indexes)} minimal solutions."

puts "Question 1 & 2:"
usage_indexes = more_performant_combos("aoc_2015_17_input1.txt", 150)
p "There are #{usage_indexes.size} solutions."
p "There are #{how_many_minimal_from_indexes(usage_indexes)} minimal solutions."

