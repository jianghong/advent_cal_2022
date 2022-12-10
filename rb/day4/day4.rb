# 2-4,6-8
# 2-3,4-5
# 5-7,7-9
# 2-8,3-7
# 6-6,4-6
# 2-6,4-8

sum = 0
File.readlines('sample.txt').each do |line|
  # ["2-4", "6-8"]
  split = line.split(',')
  elf1 = split[0]
  elf2 = split[1]

  elf1_split = elf1.split('-')
  elf2_split = elf2.split('-')

  found_a_pair = false

  if elf2_split[0].to_i >= elf1_split[0].to_i &&
      elf2_split[1].to_i <= elf1_split[1].to_i
      sum +=1
      found_a_pair = true
  end

  if found_a_pair
    next
  end

  if elf1_split[0].to_i >= elf2_split[0].to_i &&
    elf1_split[1].to_i <= elf2_split[1].to_i
    sum +=1
  end
end

puts(sum)