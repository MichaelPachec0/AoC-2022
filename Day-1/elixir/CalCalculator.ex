defmodule CalCalculator do
  defp calculate_helper(input) do
    input
    |> String.splitter("\n\n", trim: true)
    |> Stream.map(fn elf -> 
      elf 
      |> String.splitter("\n")
      |> Stream.map(fn cal -> cal |> String.to_integer end)
      |> Enum.sum
    end)
  end
  def calculate_part1(input), do: input |> calculate_helper |> Enum.max
  def calculate_part2(input), do: input |> calculate_helper |> Enum.sort |> Enum.reverse |> Enum.take(3) |> Enum.sum
end
