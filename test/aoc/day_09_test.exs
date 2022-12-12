defmodule AOC.Day09Test do
  use ExUnit.Case

  import AOC.Day09

  def input do
    [
      "R 4",
      "U 4",
      "L 3",
      "D 1",
      "R 4",
      "D 1",
      "L 5",
      "R 2"
    ]
  end

  test "part01" do
    assert part01(input()) == 13
  end

  test "part02" do
    assert part02(input()) == 8
  end
end
