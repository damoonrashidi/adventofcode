load "Char";
load "Int";
load "List";

fun looksay x:xs =
  let
    fun looksay' ([], count, number, output) = concat([output, Int.toString(count), Char.toString(number)])
      | looksay' (x::xs, count, number, output) =
        if x = number then
          output ^ Char.toString(number) ^ Char.toString(number)
        else
          output ^ Int.toString(1) ^ Char.toString(number)
  in
    looksay' (explode(s), 0, List.nth(s,0), "")
  end;