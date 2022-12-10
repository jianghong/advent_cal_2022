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

elf_groups = []
counter = 1
group = []
File.readlines('input.txt').each do |line|
  if counter < 4
    group << line
  else
    elf_groups << group
    counter = 1
    group = [line]
  end
  counter += 1
end

elf_groups << group
# elf_groups.each_with_index do |group, index|
#   puts index
#   puts group
# end

badges = []
elf_groups.each do |group|
  first_rucksack = group[0]
  second_rucksack = group[1]
  third_rucksack = group[2]

  # vJrwpWtwJgWrhcsFMMfFFhFp
  # jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
  # PmmdzqPrVvPwwTWBwg
  first_rucksack.each_char do |first_letter|
    if second_rucksack.include?(first_letter)
      if third_rucksack.include?(first_letter)
        if first_letter.match(/[a-zA-Z]/)
          badges << first_letter
          break
        end
      end
    end
  end
end

# puts badges
sum = 0
badges.each_with_index do |badge, index|
  sum += find_priority(badge)
end

puts badges.size
puts(sum)