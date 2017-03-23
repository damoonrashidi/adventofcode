class Integer
  def u16
    if self < 0
      return self + 0x10000
    elsif self > 65536
      return (self - 0x10000).u16
    else
      return self
    end
  end
end

class String
  def skip? symbols
    skip = false
    self.inputs.each do |i|
      skip = true if symbols[i] == nil
    end
    return skip
  end
  def inputs
    self.split('->')[0].scan(/[a-z]{1,2}/)
  end
  def dest
    self.split('->')[1].scan(/[a-z]{1,2}/)[0]
  end
end

instructions = File.read("input.txt").split("\n")
op_exp    = Regexp.new /([LSHIFT|RSHIFT|AND|OR|NOT]+)/
wires_exp = Regexp.new /[a-z]{1,2}/
digit_exp = Regexp.new /[\d]+/
symbols = Hash.new

wires = instructions.join("\n").scan wires_exp

nils = wires.length

while nils > 0
  instructions.each do |instruction|
    if instruction.skip? symbols
      puts "skipping #{instruction}"
      next
    else
      puts "EVAL #{instruction}"
    end
  end
end


# instructions.each do |instruction|
#   op = instruction.match(operation).to_s
#   w = instruction.scan(wires)
#   vals = instruction.scan(digit).map(&:to_i)
#   case op
#   when 'NOT'
#     symbols[w[1]] = (~symbols[w[0]]).u16
#   when 'AND'
#     if vals.length > 0
#       symbols[w[1]] = (vals[0] & symbols[w[0]]).u16
#     else
#       symbols[w[2]] = (symbols[w[0]] & symbols[w[1]]).u16
#     end
#   when 'OR'
#     symbols[w[2]] = (symbols[w[0]] | symbols[w[1]]).u16
#   when 'RSHIFT'
#     symbols[w[1]] = (symbols[w[0]] >> vals[0]).u16
#   when 'LSHIFT'
#     symbols[w[1]] = (symbols[w[0]] << vals[0]).u16
#   else
#     symbols[w[0]] = w[1].nil? ? vals[0] : symbols[w[1]].u16
#   end
# end
