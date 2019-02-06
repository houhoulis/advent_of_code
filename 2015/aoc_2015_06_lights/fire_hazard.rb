class Grid
  OFF, ON = 0, 1
  attr_accessor :lights

  def initialize rows, columns, on_off = OFF
    @lights = (1..rows).map { Array.new columns, on_off }
  end

  def turn_off row1, column1, row2, column2
    set row1, column1, row2, column2, ->(_) { OFF }
  end

  def turn_on row1, column1, row2, column2
    set row1, column1, row2, column2, ->(_) { ON }
  end

  def toggle row1, column1, row2, column2
    set row1, column1, row2, column2, ->(x) { (x + 1) % 2 }
  end

  def ancient_nordic_elvish_turn_off row1, column1, row2, column2
    set row1, column1, row2, column2, ->(x) { [x - 1, OFF].max }
  end

  def ancient_nordic_elvish_turn_on row1, column1, row2, column2
    set row1, column1, row2, column2, ->(x) { x + 1 }
  end

  def ancient_nordic_elvish_toggle row1, column1, row2, column2
    set row1, column1, row2, column2, ->(x) { x + 2 }
  end

  def set row1, column1, row2, column2, prok
    (row1..row2).each do |row|
      (column1..column2).each do |column|
        lights[row][column] = prok.call(lights[row][column])
      end
    end
  end

  def sum
    lights.flatten.inject(:+)
  end
end

class FireHazard
  def perform width, height, instruction_file
    grid = Grid.new width, height
    instructions(instruction_file).each do |meth, row1, column1, row2, column2|
      grid.send meth, row1, column1, row2, column2
    end
    grid.sum
  end

  def perform_2 width, height, instruction_file
    grid = Grid.new width, height
    instructions(instruction_file).each do |meth, row1, column1, row2, column2|
      meth = 'ancient_nordic_elvish_' + meth.to_s
      grid.send meth, row1, column1, row2, column2
    end
    grid.sum
  end

  def instructions instruction_file
    File.readlines(instruction_file).map do |line|
      match_data = /(([a-z]+ )+)(\d+),(\d+)\D+(\d+),(\d+)/.match(line)
      verb = case match_data[1]
              when "turn off "
                :turn_off
              when "turn on "
                :turn_on
              when "toggle "
                :toggle
              else
                raise "unrecognized instruction: '#{verb}'!"
              end
      [verb, match_data[3].to_i, match_data[4].to_i, match_data[5].to_i, match_data[6].to_i]
    end
  end
end
