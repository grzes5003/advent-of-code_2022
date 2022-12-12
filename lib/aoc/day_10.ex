defmodule AOC.Day10 do

  @instructions [:noop, :addx, :sleep]

  def score(register, idx) do
    case idx do
      idx when idx in [20, 60, 100, 140, 180, 220] -> idx * register
      _ -> 0
    end
  end

  def get_instruction(item) do
    case item do
      [_, val] -> {:addx, String.to_integer(val)}
      _ -> {:noop}
    end
  end

  def cycle(_, [], _, score), do: score
  def cycle(register, instructions, idx \\ 1, score \\ 0) do
    case (instructions |> hd) do
      {:addx, val} -> cycle(register, [ {:sleep, val} | instructions |> tl] , idx + 1, score + score(register, idx))
      {:sleep, val} -> cycle(register + val, instructions |> tl , idx + 1, score + score(register, idx))
      {:noop} -> cycle(register, instructions |> tl, idx + 1, score + score(register, idx))
    end
  end

  def parse(list) do
    list
    |> Enum.map(fn line -> String.split(line, " ", trim: true) end)
    |> Enum.map(&get_instruction/1)
  end

  def part01(args) do
    instr = parse(args)
    cycle(1, instr)
  end

  def part02(args) do
  end
end
