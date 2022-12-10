# Open the input file
File.open("sample.txt") do |file|
  # Read the list of rucksack contents from the input file
  contents = []
  while line = file.gets
    contents << line.strip
  end

  # Calculate the sum of the priorities of the item types that appear
  # in both compartments of each rucksack
  sum = 0
  contents.each do |rucksack|
    # Split the rucksack into its two compartments
    comp1, comp2 = rucksack.split('').each_slice((rucksack.length / 2.0).round).to_a

    # Find the item type that appears in both compartments
    common = (comp1 & comp2).first

    # Calculate the priority of the item type
    priority = common.ord - 'a'.ord + 1
    priority += 26 if common.match(/[A-Z]/)

    # Add the priority to the sum
    sum += priority
  end

  # Print the sum of the priorities
  puts sum
end
