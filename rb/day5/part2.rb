stacks = [
  %w(N Z),
  %w(D C M),
  %w(P)
]

stacks = [
  %w(W P G Z V S B),
  %w(F Z C B V J),
  %w(C D Z N H M L V),
  %w(B J F P Z M D L),
  %w(H Q B J G C F V),
  %w(B L S T Q F G),
  %w(V Z C G L),
  %w(G L N),
  %w(C H F J),
]

File.readlines('input.txt').each do |line|
  matched_data = line.match(/move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)/)
  quantity = matched_data[1].to_i
  from_stack = matched_data[2].to_i
  to_stack = matched_data[3].to_i

  crates = stacks[from_stack-1].shift(quantity)
  stacks[to_stack-1] = crates.concat(stacks[to_stack-1])
end

sum = ""
stacks.each do |stack|
  sum = sum.concat(stack[0])
end

p sum