defmodule AOC.Day06Test do
  use ExUnit.Case

  import AOC.Day06

  def input do
    [
      {"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19},
      {"bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23},
      {"nppdvjthqldpwncqszvftbrmjlhg", 6, 23}
    ]
  end

  test "part01" do
    for {item, val, _} <- input(), do: assert part01(item) == val
  end

  test "part02" do
    for {item, _, val} <- input(), do: assert part02(item) == val
  end
end
