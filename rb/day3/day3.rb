def find_priority(letter)
  priorities = {
    a: 1,
    b: 2,
    c: 3, 
    d: 4,
    e: 5,
    f: 6,
    g: 7,
    h: 8,
    i: 9,
    j: 10,
    k: 11,
    l: 12,
    m: 13,
    n: 14,
    o: 15,
    p: 16,
    q: 17,
    r: 18,
    s: 19,
    t: 20,
    u: 21,
    v: 22,
    w: 23,
    x: 24,
    y: 25,
    z: 26
  }
  priority = priorities[letter.to_sym]
  if priority == nil
    downcase_letter = letter.downcase
    return priorities[downcase_letter.to_sym] + 26
  end
  return priority
end

matching_priorities = []
File.readlines('input.txt').each do |line|
  count = line.size
  half = count / 2
  first_compartment = line[0..(half - 1)]
  second_compartment = line[half..-1]
  
  found = false
  first_compartment.each_char do |first_letter|
    second_compartment.each_char do |second_letter|
      if !found
        if first_letter == second_letter
          found = true
          matching_priorities << first_letter
        end
      end
    end
  end
end

sum = 0
matching_priorities.each do |letter|
  sum += find_priority(letter)
end

puts(matching_priorities.size)
puts(sum)

