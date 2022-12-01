defmodule AOC.Day01 do
  def get_chunks(list) do
    list
    |> Enum.chunk_by(fn x -> x != "" end)
    |> Enum.reject(fn x -> x == [""] end)
  end

  def parse(list) do
    list |> Enum.map(fn sub -> sub |> Enum.map(&String.to_integer/1) end)
  end

  def part01(args) do
    parse(get_chunks(args))
    |> Enum.map(fn x -> Enum.sum(x) end)
    |> Enum.reduce(&max/2)
  end

  def part02(args) do
    parse(get_chunks(args))
    |> Enum.map(fn x -> Enum.sum(x) end)
    |> Enum.sort(:desc)
    |> Enum.take(3)
    |> Enum.sum()
  end
end
