input = File.readlines('input.txt').map{|l| l.split(' ').map{|i| i.to_i}} 
p input.select{|s| s.max < (s.reduce(:+) - s.max)}.length
