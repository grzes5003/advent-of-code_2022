defmodule AOC.Day04 do
  def parse_range(list) do
    list
    |> String.split("-", trim: true)
    |> (fn [left, right] -> {String.to_integer(left), String.to_integer(right)} end).()
  end

  def parse(list) do
    list
    |> Enum.map(fn sub -> String.split(sub, ",", trim: true) end)
    |> Enum.map(fn [left, right] -> {parse_range(left), parse_range(right)} end)
  end

  def part01(args) do
    args
    |> parse()
    |> Enum.filter(fn {{a, b}, {c, d}} ->
      (c <= a && d >= b) ||
        (a <= c && b >= d)
    end)
    |> Enum.count()
  end

  def part02(args) do
    args
    |> parse()
    |> Enum.filter(fn {{a, b}, {c, d}} ->
      (b >= c && a <= c) ||
      (a <= d && c <= b)
    end)
    |> Enum.count()
  end
end
