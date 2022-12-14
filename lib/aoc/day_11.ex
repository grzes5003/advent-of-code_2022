defmodule AOC.Day11 do

  defmodule Monkey do
    use Agent

    defstruct id: 0, items: [], op: 1, div_by: 1, addr: {0, 0}, active: 0

    def start_link(id, initial_value) do
      Agent.start_link(fn -> initial_value end, name: id)
    end

    def add_item(id, value) do
      Agent.update(id, &(%{&1 | items: &1.items ++ [value] }))
    end

    def handle(id) do
      Agent.update(id, &handle_monkey/1)
    end

    def handle_monkey(monkey) do
      {true_dest, false_dest} = monkey.items
      |> Enum.map(fn val -> (
        case monkey.op do
          {op, "old"} -> apply Kernel, op, [val, val]
          {op, num} -> apply Kernel, op, [val, String.to_integer(num)]
        end)  end)
      |> Enum.map(&(Kernel.trunc(&1 / 3)))
      |> Enum.split_with(&(rem(&1, monkey.div_by) == 0))

      true_dest
      |> Enum.each(fn item -> Monkey.add_item(monkey.addr |> elem(0), item) end)

      false_dest
      |> Enum.each(fn item ->Monkey.add_item(monkey.addr |> elem(1), item) end)

      %{monkey | items: [], active: monkey.active + length(monkey.items) }
    end

  end

  def get_chunks(list) do
    list
    |> Enum.chunk_by(fn x -> x != "" end)
    |> Enum.reject(fn x -> x == [""] end)
  end

  def parse_monkey(list) do
    str = list
    |> Enum.map(&String.trim/1)

    id = Enum.at(str, 0)
      |> String.split("", trim: true)
      |> Enum.at(-2) |> String.to_integer()

    items = Enum.at(str, 1)
      |> String.split(":")
      |> Enum.at(1)
      |> String.split(", ", trim: true)
      |> Enum.map(&String.trim/1)
      |> Enum.map(&String.to_integer/1)

    op = Enum.at(str, 2)
      |> String.split(" ")
      |> (&({String.to_atom(Enum.at(&1, -2)), Enum.at(&1, -1)})).()
      |> IO.inspect

    div_by = Enum.at(str, 3)
    |> String.split( " ")
    |> Enum.at( -1)
    |> String.to_integer

    [true_addr, false_addr] = Enum.slice(str, 4, 2)
      |> Enum.map(&(String.split(&1, " ")))
      |> Enum.map(&(Enum.at(&1, -1)))
      |> Enum.map(&String.to_atom/1)

    %Monkey{
      id: id,
      items: items,
      op: op,
      div_by: div_by,
      addr: {true_addr, false_addr}
    }
  end



  def shout(ids, 0), do: ids
  def shout(ids, left) do
    ids
    |> Enum.each(fn id -> Monkey.handle(id) end)
    shout(ids, left - 1)
  end

  def parse(list) do
    list
    |> get_chunks
    |> Enum.map(&parse_monkey/1)
    |> Enum.map(fn monkey ->
      Monkey.start_link(
        monkey.id
        |> Integer.to_string
        |> String.to_atom, monkey)
    end)
  end

  def calculate(args, limit) do
    agents = Enum.to_list(0..length(parse(args))-1)
    |> Enum.map(&Integer.to_string/1)
    |> Enum.map(&String.to_atom/1)
    shout(agents, limit)

    [a,b] = agents
    |> Enum.map(fn id -> Agent.get(id, & &1.active) end)
    |> Enum.sort
    |> Enum.reverse
    |> Enum.take(2)
    a * b
  end

  def part01(args) do
    # calculate(args, 20)
  end

  def part02(args) do
    calculate(args, 1)
  end
end
