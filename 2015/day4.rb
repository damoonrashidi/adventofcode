require 'digest'
base = "ckczppom"
md5 = Digest::MD5.new
found = false
input = 0
while !found do
  bits = md5.hexdigest(base + input.to_s)[0..4]
  if bits == "00000"
    print "Bits: #{bits} - #{md5.hexdigest}\n#{input}\n"
    found = true
  end
  input += 1
end