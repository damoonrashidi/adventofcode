input = File.readlines('input.txt').map{|line| line.sub(/\n/, '')}

tests = {
  'aaaaa-bbb-z-y-x-123[abxyz]' => true,
  'a-b-c-d-e-f-g-h-987[abcde]' => true,
  'not-a-real-room-404[oarel]' => true,
  'totally-real-room-200[decoy]' => false
}

def count(room)
  hsh = {}
  room.split('').each do |letter|
    hsh[letter] = hsh[letter].nil? ? 1 : hsh[letter] + 1
  end
  return hsh.sort_by{|k,v| -v}.to_h
end

def run (input)
  room = input.match(/([a-z\-])+/)[0].slice(0..-2).gsub('-','')
  value = input.match(/([\d])+/)[0].to_i
  checksum = input.match(/\[([a-z])+\]/)[0]
  c = count(room)
  for i in (0..checksum.length)
    if checksum.split('')[i] == c[i].key then
      return value
    else
      return 0
    end
  end
end

if ARGV[0] == 'test'
  tests.each do |input, outcome|
    puts outcome == run(input) ? "PASSED" : "FAILED"
  end
end