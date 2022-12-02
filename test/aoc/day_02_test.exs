defmodule AOC.Day02Test do
  use ExUnit.Case

  import AOC.Day02

  def input do
    ["A Y", "B X", "C Z"]
  end

  test "part01" do
    assert part01(input()) == 15
  end

  test "part02" do
    assert part02(input()) == 12
  end
end