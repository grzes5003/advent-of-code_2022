defmodule AOC.Day05 do
  defmodule Instr do
    defstruct [:quant, :src, :trg]
  end

  @variant [:reverse, :classic]

  def parse_stack(list) do
    list
    |> Enum.map(fn line -> String.split(line, "", trim: true) end)
    |> Enum.map(fn list -> Enum.chunk_every(list, 3, 4, []) end)
    |> Enum.map(fn list -> Enum.map(list, fn [_, item, _] -> [item] end) end)
    |> Enum.zip_with(& &1)
    |> Enum.map(fn list -> List.flatten(list) end)
    |> Enum.map(fn list -> Enum.filter(list, fn item -> item != " " end) end)
    |> Enum.map(fn list ->
      {String.to_integer(List.last(list)), list |> Enum.reverse() |> tl() |> Enum.reverse()}
    end)
    |> Map.new()
  end

  def parse_instruction(line) do
    line
    |> String.split(" ")
    |> (fn [_, quant, _, from, _, to] ->
          %Instr{
            quant: String.to_integer(quant),
            src: String.to_integer(from),
            trg: String.to_integer(to)
          }
        end).()
  end

  def parse_instructions(list) do
    list
    |> Enum.map(fn l -> parse_instruction(l) end)
  end

  def parse(list) do
    list
    |> Enum.chunk_by(fn x -> x != "" end)
    |> (fn [l, _, r] -> [parse_stack(l), parse_instructions(r)] end).()
  end

  def move(stacks, instructions, variant) when hd(instructions).quant > 0 do
    instruction = instructions |> hd

    {to, altered} = Enum.split(stacks[instruction.src], instruction.quant)

    to =
      case variant do
        :reverse -> Enum.reverse(to)
        :classic -> to
      end

    stacks =
      stacks
      |> Map.replace!(instruction.src, altered)
      |> (&Map.replace!(&1, instruction.trg, to ++ &1[instruction.trg])).()

    move(stacks, instructions |> tl(), variant)
  end

  def move(stacks, [], _), do: stacks

  def part01(args) do
    [items, instr] = parse(args)

    move(items, instr, :reverse)
    |> Enum.map(fn {_, val} -> hd(val) end)
    |> List.to_string()
    |> IO.inspect()
  end

  def part02(args) do
    [items, instr] = parse(args)

    move(items, instr, :classic)
    |> Enum.map(fn {_, val} -> hd(val) end)
    |> List.to_string()
    |> IO.inspect()
  end
end
