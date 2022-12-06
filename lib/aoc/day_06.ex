defmodule AOC.Day06 do

  def find(string, window) do
    String.split(string, "", trim: true)
    |> Enum.with_index()
    |> Enum.chunk_every(window, 1, :discard)
    |> Enum.map(fn pairs ->
      Enum.uniq_by(pairs, fn {k, _} -> k end)
    end)
    |> Enum.find(fn pairs -> length(pairs) == window end)
    |> List.last |> elem(1) |> Kernel.+(1)
  end

  def part01(args) do
    find(args, 4)
  end

  def part02(args) do
    find(args, 14)
  end
end
