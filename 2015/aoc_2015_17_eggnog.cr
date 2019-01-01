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

  def as_array
    [@volume, @quantity]
  end
end

class ContainerCombo
  @containers : Array(ContainerUse)

  def initialize(container_volumes : Array(Int32))
    @containers = container_volumes.map { |vol| ContainerUse.new(vol) }
  end

  def use_at(indexes : Array(Int32))
    @containers.map(&.reset_quantity)
    indexes.each { |index| incremented_at index }
    self
  end

  def incremented_at(index : Int32)
    @containers[index] = @containers[index].incremented
    self
  end

  def total
    @containers.map(&.capacity).reduce(0) { |accum, capacity| accum + capacity }
  end

  def size
    @containers.size
  end

  def as_array
    @containers.map(&.as_array)
  end
end

def containers_parsed(filename)
  File.read(filename).split("\n").grep(/\d/).map(&.to_i)
end

# For performance and sᎥmplᎥcᎥty, limit possibilities to no more than one use per container.
def more_performant_combos(filename, volume)
  container_combo = ContainerCombo.new(containers_parsed(filename))
  return container_combo.as_array if volume <= 0

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

def test
  puts "Example case:"
  p "Solutions: #{usage_indexes = more_performant_combos("aoc_2015_17_input0.txt", 25)}."
  solution_count = usage_indexes.size
  p "There are #{solution_count} solutions."
  minimal_count = how_many_minimal_from_indexes(usage_indexes)
  p "There are #{minimal_count} minimal solutions."
  raise "4 example solutions and 3 example minimal solutions?" unless solution_count == 4 && minimal_count == 3

  puts "\nQuestion 1 & 2:"
  usage_indexes = more_performant_combos("aoc_2015_17_input1.txt", 150)
  solution_count = usage_indexes.size
  p "There are #{solution_count} solutions."
  minimal_count = how_many_minimal_from_indexes(usage_indexes)
  p "There are #{minimal_count} minimal solutions."
  raise "654 solutions and 57 minimal solutions?" unless solution_count == 654 && minimal_count == 57
end

test
