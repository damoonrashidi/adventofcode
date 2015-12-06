#!/usr/bin/ruby
class String

  def nice1?
    return self.match(/([a-z])\1+/) != nil &&
      self.match(/ab|cd|pq|xy/) == nil &&
      self.split('').map{|l| "aeoui".include?(l)}.count(true) >= 3
  end

  def nice2?
    return self.match(/([a-z])([^\1])\1/) != nil &&
      self.match(/([a-z][a-z])(.+|)\1/) != nil
  end
end


words = File.read("input.txt").split("\n")
puts "Part 1: " + words.map{|word| word.nice1? }.count(true).to_s
puts "Part 2: " + words.map{|word| word.nice2? }.count(true).to_s
