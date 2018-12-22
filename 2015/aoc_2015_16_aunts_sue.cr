EXACT_PROPS = {
  "children" => 3,
  "samoyeds" => 2,
  "akitas" => 0,
  "vizslas" => 0,
  "cars" => 2,
  "perfumes" => 1
}

LOWER_LIMIT_PROPS = {
  "cats" => 7,
  "trees" => 3
}

UPPER_LIMIT_PROPS = {
  "pomeranians" => 3,
  "goldfish" => 5,
}

def matches_exact_props(sue)
  (EXACT_PROPS.keys & sue.keys).none? { |k| sue[k].to_i != EXACT_PROPS[k] }
end

def exceeds_lower_limit_props(sue)
  (LOWER_LIMIT_PROPS.keys & sue.keys).none? { |k| sue[k].to_i <= LOWER_LIMIT_PROPS[k] }
end

def below_upper_limit_props(sue)
  (UPPER_LIMIT_PROPS.keys & sue.keys).none? { |k| sue[k].to_i >= UPPER_LIMIT_PROPS[k] }
end

def detect_a_sue(sues)
  matches = sues.select do |sue|
    matches_exact_props(sue) && exceeds_lower_limit_props(sue) && below_upper_limit_props(sue)
  end
  raise "hello! #{matches.size}" unless matches.size == 1

  matches.first
end

def solve
  sues = parse_input("aoc_2015_16_input.txt")
  p detect_a_sue(sues)
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
