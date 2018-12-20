def detect_a_sue(sues, props)
  matches = sues.select do |sue|
    props.map { |k, v| v == sue[k] if sue.keys.includes?(k) }.compact.uniq == [true]
  end
  raise "hello! #{matches.size}" unless matches.size == 1

  matches.first
end

def solve
  sues = parse_input("aoc_2015_16_input.txt")
  props = {
    "children" => "3",
    "cats" => "7",
    "samoyeds" => "2",
    "pomeranians" => "3",
    "akitas" => "0",
    "vizslas" => "0",
    "goldfish" => "5",
    "trees" => "3",
    "cars" => "2",
    "perfumes" => "1",
  }
  p detect_a_sue(sues, props)
end

def parse_input(filename)
  File.read(filename).split("\n").reduce([] of Hash(String, String)) do |sues, entry|
    if entry.size < 8 # e.g. empty line
      sues
    else
      new_sue = { "number" => entry.split[1][0..-2] }
      remaining = entry[6..-1]
      regex = /\w+: \d+/
      while match = regex.match(remaining)
        compound, value = match[0].split(": ")
        new_sue[compound] = value
        remaining = match.post_match
      end
      sues + [new_sue]
    end
  end
end

solve()
