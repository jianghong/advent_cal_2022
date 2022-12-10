elves = []

sum_cal = 0

File.readlines('input.txt').each do |line|
  if line.length > 1
    sum_cal += line.to_i
  else
    elves << sum_cal
    sum_cal = 0
  end
end

elves << sum_cal


# [6000, 4000, 11000, 24000, 10000]
# puts(elves)

sorted_elves = elves.sort
# puts(sorted_elves)
puts("highest calories found: " + sorted_elves.last.to_s)

top_3_cals = sorted_elves[-1] + sorted_elves[-2] + sorted_elves[-3]
puts("top 3 calories: " + top_3_cals.to_s)