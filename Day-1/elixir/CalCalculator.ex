defmodule CalCalculator do
  def calculate(input) do
    input
    |> String.splitter("\n\n", trim: true)
    |> Stream.map(fn elf ->
      elf
      |> String.splitter("\n")
      |> Stream.map(fn cal -> cal |> String.to_integer() end)
      |> Enum.sum()
    end)
    |> Enum.max()
  end
end
