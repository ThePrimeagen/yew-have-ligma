

Enum.reduce(1..5, 0, &Kernel.+/2) |> IO.inspect()

Enum.reduce 1..5,
     0,
   &Kernel.+/2
