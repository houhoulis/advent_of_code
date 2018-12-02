class Guest
  attr_accessor :name, :preferences
  def initialize name
    @name = name
    @preferences = {}
  end
  def add_preference other, points
    preferences[other] = points
  end
  def preference other
    return 0 if name == self.class.hostname || other == self.class.hostname
    preferences[other]
  end
  def self.hostname
    'A. Most Unique Name'
  end
end

def guest_preferences filename = 'aoc_2015_13_input.txt'
  raise "no input?" unless File.file?(filename)
  guests = []
  File.readlines(filename).each do |line|
    name, points, neighbor = parse(line)
    guest = guests.detect { |g| g.name == name }
    unless guest
      guests << guest = Guest.new(name)
    end
    guest.add_preference neighbor, points
  end
  guests
end

def guest_preferences_plus_host filename = 'aoc_2015_13_input.txt'
  guests = guest_preferences filename
  raise "Is the host also a guest?" if guests.any? { |guest| guest.name == Guest.hostname }
  guests << Guest.new(Guest.hostname)
end

def parse line
  name, gain_lose, quantity, neighbor = line.split(' ').values_at(0, 2, 3, -1)
  points = gain_lose == "gain" ? quantity.to_i : 0 - quantity.to_i
  neighbor.chop!
  [name, points, neighbor]
end

def happiness_score guests
  (guests + [guests.first]).each_cons(2).map do |g1, g2|
    g1.preference(g2.name) + g2.preference(g1.name)
  end.inject(0, :+)
end

def most_happiness guests
  guests.permutation.to_a.map { |arrangement| happiness_score arrangement }.max
end
