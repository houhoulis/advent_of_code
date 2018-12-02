class Password
  attr_accessor :str
  def initialize str
    @str = str
  end

  def valid?
    correct_format? && has_no_bad_letters? && has_sequence? && has_two_doubles?
  end

  def correct_format?
    str.length == 8 && (str.chars - ('a'..'z').to_a).empty?
  end

  def has_two_doubles?
    dupe_char_1, dupe_index_1 = str.chars.each_with_index.detect { |char, i| char == str[i + 1] }
    return false unless dupe_char_1

    remainder = str[(dupe_index_1 + 2)..-1]
    remainder.chars.each_with_index.any? { |char, i| char != dupe_char_1 && char == remainder[i + 1] }
  end

  def has_no_bad_letters?
    str.exclude?('i') && str.exclude?('o') && str.exclude?('l')
  end

  def has_sequence?
    str.chars.each_cons(3).any? { |a, b, c| b == a.succ && c == b.succ }
  end

  def increment
    @str = bounded_next
    while !valid?
      @str = bounded_next
    end
  end

  def bounded_next
    return 'aaaaaaaa' if str == 'zzzzzzzz'

    bad_char, bad_index = str.chars.each_with_index.detect { |char, _| char.in? %w(i o l) }
    return str.succ unless bad_char

    str[0, bad_index] + bad_char.succ + 'a' * (8 - (bad_index + 1))
  end
end
