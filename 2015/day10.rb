require 'parallel'

def looksay list
  ls list, 0, list[0], ""
end

def ls list, count, value, output
 
  #we are at the last item, tally it up and stop
  if list.length == 1
    if list[0] == value
      return "#{output}#{count+1}#{list[0]}"
    else
      return "#{output}#{count}#{value}1#{list[0]}"
    end
  end

  #check if we are increasing the counter or if we are resetting it
  if list[0] == value
    return ls(list[1..list.length-1], count+1, value, output)
  else
    return ls(list[1..list.length-1], 1, list[0], "#{output}#{count}#{value}")
  end
end

input = "4819211"
for i in (0..32)
  input = input.split("").each_slice(4).to_a
  input = Parallel.map(input, :in_processes => 4){ |c| looksay(c) }
  input = input.reduce(:+)
end
puts input.length


#try to devide the input in chunks, then run the ls function on each chunk then merge the output of each chunk