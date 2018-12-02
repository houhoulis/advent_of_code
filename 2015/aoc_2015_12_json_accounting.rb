def explode obj
  return obj.flat_map {|thingee| explode thingee } if obj.is_a?(Array)
  return obj unless obj.is_a?(Hash)
  obj.keys.flat_map{|thingee| explode thingee } + obj.values.flat_map{|thingee| explode thingee }
end

def sum_numbers string
  Array(explode(JSON.parse string)).select {|item| item.is_a? Integer }.reduce(0, :+)
end

def explode_without_red obj
  return obj.flat_map {|thingee| explode_without_red thingee } if obj.is_a?(Array)
  return obj unless obj.is_a?(Hash)
  return 0 if obj.value?("red")
  obj.keys.flat_map{|thingee| explode_without_red thingee } + obj.values.flat_map{|thingee| explode_without_red thingee }
end

def sum_numbers_without_red string
  Array(explode_without_red(JSON.parse string)).select {|item| item.is_a? Integer }.reduce(0, :+)
end
