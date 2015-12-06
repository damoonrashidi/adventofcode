require 'digest'
base = "ckczppom"
md5 = Digest::MD5.new
(0..100000000).each { |input|
  bits = md5.hexdigest(base + input.to_s)[0..4]
  if bits == "00000"
    print "Bits: #{bits} - #{md5.hexdigest}\n#{input}\n"
    break
  end
}