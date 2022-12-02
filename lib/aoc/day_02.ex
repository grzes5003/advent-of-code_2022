defmodule AOC.Day02 do

  @choices [:rock, :paper, :scissors]

  def value(:rock), do: 1
  def value(:paper), do: 2
  def value(:scissors), do: 3

  def trans(char) do
    case char do
      a when a in ["A", "X"] -> :rock
      a when a in ["B", "Y"] -> :paper
      a when a in ["C", "Z"] -> :scissors
    end
  end

  def get_outcome(:rock, :scissors), do: 6 + value(:rock)
  def get_outcome(:paper, :rock), do: 6 + value(:paper)
  def get_outcome(:scissors, :paper), do: 6 + value(:scissors)

  def get_outcome(val, val), do: 3 + value(val)
  def get_outcome(a, _), do: value(a)

  def parse(list) do
    list
    |> Enum.map(fn sub -> String.split(sub, " ", trim: true) end)
    |> Enum.map(fn [a,b] -> {trans(a), trans(b)} end)
  end

  def wrap({a,b}) do
    get_outcome(b,a)
  end

  def part01(args) do
    args |> parse() |>
    Enum.map(fn item -> wrap(item) end)
    |> Enum.sum()
  end


  ############################
  @choices [:win, :loose, :draw]

  def trans_part02(char) do
    case char do
      "X" -> :loose
      "Y" -> :draw
      "Z" -> :win
    end
  end

  def parse_part02(list) do
    list
    |> Enum.map(fn sub -> String.split(sub, " ", trim: true) end)
    |> Enum.map(fn [a,b] -> {trans(a), trans_part02(b)} end)
  end

  def wrap_part02({a,b}) do
    limit = case b do
      :win -> 7..10
      :loose -> 1..3
      :draw -> 4..6
    end
    a = [get_outcome(:rock, a),
      get_outcome(:paper, a),
      get_outcome(:scissors, a)]
    Enum.filter(a, fn x -> x in limit end)
    |> Enum.at(0)
  end

  def part02(args) do
    args |> parse_part02()
    |> Enum.map(fn item -> wrap_part02(item) end)
    |> Enum.sum()
  end
end