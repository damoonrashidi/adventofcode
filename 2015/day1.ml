fun map f [] = []
  | map f (x::xs) = (f x) :: map f xs;

fun dir(#"(") = 1
  | dir(#")") = ~1
  | dir _ = 0;

fun solve building =
  let
    fun solve' (floor, []) = floor
      | solve' (floor, x::xs) = solve' (floor+x, xs)
  in
    solve' (0, building)
  end;
