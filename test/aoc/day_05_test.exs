defmodule AOC.Day04Test do
  use ExUnit.Case

  import AOC.Day05

  def input do
    [
      "    [D]    ",
      "[N] [C]    ",
      "[Z] [M] [P]",
      " 1   2   3 ",
      "",
      "move 1 from 2 to 1",
      "move 3 from 1 to 3",
      "move 2 from 2 to 1",
      "move 1 from 1 to 2"
    ]
  end

  test "part01" do
    assert part01(input()) == "CMZ"
  end

  test "part02" do
    assert part02(input()) == 4
  end
end
