# Part 1 Map
# Rock = A = X = 1
# Paper = B = Y = 2
# Scissor = C = Z = 3

# Part 2 Map
# Rock = A = 1
# Paper = B = 2
# Scissor = C = 3

# Part 2 Map
# Loss = X = 0
# Draw = Y = 3
# Win =  Z = 6

def calculate_points2(enemy, player)
  points = 0
  if player == "X" # Loss
    points += 0
    if enemy == "A" # Rock
      points += 3 # Scissor
    elsif enemy == "B" # Paper
      points += 1 # Rock
    elsif enemy == "C" # Scissor
      points += 2 # Paper
    end
  elsif player == "Y" # Draw
    points += 3
    if enemy == "A" # Rock
      points += 1
    elsif enemy == "B" # Paper
      points += 2
    elsif enemy == "C" # Scissor
      points += 3
    end
  elsif player == "Z" # Win
    points += 6
    if enemy == "A" # Rock
      points += 2
    elsif enemy == "B" # Paper
      points += 3
    elsif enemy == "C" # Scissor
      points += 1
    end
  end
  return points
end

def calculate_points(enemy, player)
  points = 0
  if player == "X" # Rock
    points += 1
    if enemy == "A" # Rock
      points += 3
    elsif enemy == "B" # Paper
      points += 0
    elsif enemy == "C" # Scissor
      points += 6
    end
  elsif player == "Y" # Paper
    points += 2
    if enemy == "A" # Rock
      points += 6
    elsif enemy == "B" # Paper
      points += 3
    elsif enemy == "C" # Scissor
      points += 0
    end
  elsif player == "Z" # Scissor
    points += 3
    if enemy == "A" # Rock
      points += 0
    elsif enemy == "B" # Paper
      points += 6
    elsif enemy == "C" # Scissor
      points += 3
    end
  end
  return points
end

sum = 0
sum2 = 0
File.readlines('input.txt').each do |line|
  # A Y
  # ["A", "Y"]
  split = line.split(" ")
  enemy = split[0]
  player = split[1]
  sum +=  calculate_points(enemy, player)
  sum2 += calculate_points2(enemy, player)
end

puts(sum)
puts(sum2)