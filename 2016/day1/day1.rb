instructions = "R4, R3, L3, L2, L1, R1, L1, R2, R3, L5, L5, R4, L4, R2, R4, L3, R3, L3, R3, R4, R2, L1, R2, L3, L2, L1, R3, R5, L1, L4, R2, L4, R3, R1, R2, L5, R2, L189, R5, L5, R52, R3, L1, R4, R5, R1, R4, L1, L3, R2, L2, L3, R4, R3, L2, L5, R4, R5, L2, R2, L1, L3, R3, L4, R4, R5, L1, L1, R3, L5, L2, R76, R2, R2, L1, L3, R189, L3, L4, L1, L3, R5, R4, L1, R1, L1, L1, R2, L4, R2, L5, L5, L5, R2, L4, L5, R4, R4, R5, L5, R3, L1, L3, L1, L1, L3, L4, R5, L3, R5, R3, R3, L5, L5, R3, R4, L3, R3, R1, R3, R2, R2, L1, R1, L3, L3, L3, L1, R2, L1, R4, R4, L1, L1, R3, R3, R4, R1, L5, L2, R2, R3, R2, L3, R4, L5, R1, R4, R5, R4, L4, R1, L3, R1, R3, L2, L3, R1, L2, R3, L3, L1, L3, R4, L4, L5, R3, R5, R4, R1, L2, R3, R5, L5, L4, L1, L1".split(", ")


tests = {
  5 => ["R2", "L3"],
  2 => ["R2", "R2", "R2"],
  12 => ["R5", "L5", "R5", "R3"],
  90 => ["R5", "L85"]
}

class Navigator

  def initialize ()
    @orientation = 'N'
    @steps = {
      'N' => 0,
      'E' => 0,
      'W' => 0,
      'S' => 0
    }
  end

  def getOrientation()
    return @orientation
  end

  def setOrientation (turn)
    r = {
      'N' => 'E',
      'E' => 'S',
      'S' => 'W',
      'W' => 'N'
    }
    l = {
      'N' => 'W',
      'W' => 'S',
      'S' => 'E',
      'E' => 'N'
    }

    if turn == 'R'
      @orientation = r[@orientation]
    else
      @orientation = l[@orientation]
    end

  end

  def walk (steps)
    @steps[@orientation] += steps
  end

  def getDistance
    return (@steps["W"]-@steps["E"]).abs() + (@steps["N"]-@steps['S']).abs()
  end

  def followMap(map)
    map.each do |instruction|
      self.setOrientation(instruction[0])
      self.walk(instruction[1..-1].to_i)
    end
  end

end


if ARGV[0] == 'test' then
  tests.each do |distance, map|
    navigator = Navigator.new()
    navigator.followMap(map)
    puts distance == navigator.getDistance() ? "PASSED" : "FAILED (expected: #{distance}, result: #{navigator.getDistance()})"
  end
else
  navigator = Navigator.new()
  navigator.followMap(instructions)
  puts navigator.getDistance()
end

