defmodule AOC.Day03 do
  @spec parse(binary) :: {binary, binary}
  def parse(line) do
  len = round(String.length(line) / 2)
  String.split_at(line, len)
  end

  def count(bag) do
    String.to_charlist(bag)
    |> Enum.reduce(%{}, fn char, acc ->
      Map.put(acc, char, (acc[char] || 0) + 1) end)
  end

  def occurs(bag) do
    String.to_charlist(bag)
    |> MapSet.new()
  end

  def value(char) when char < 97 , do: char - 65 + 27
  def value(char), do: char - 96

  @spec part01(any) :: number
  def part01(args) do
    Enum.map(args, &parse/1)
    |> Enum.map(fn {a,b} -> {occurs(a), occurs(b)} end)
    |> Enum.map(fn {a,b} -> MapSet.intersection(a,b) end)
    |> Enum.map(fn a -> Enum.at(MapSet.to_list(a), 0) end)
    |> Enum.map(fn char -> value(char) end)
    |> Enum.sum()
  end

  @spec part02(any) :: number
  def part02(args) do
    Enum.map(args, &occurs/1)
    |> Enum.chunk_every(3)
    |> Enum.map(fn sets ->  Enum.reduce(sets, fn set, acc -> MapSet.intersection(acc, set)  end) end)
    |> Enum.map(fn a -> Enum.at(MapSet.to_list(a), 0) end)
    |> Enum.map(fn char -> value(char) end)
    |> Enum.sum()
  end
end
