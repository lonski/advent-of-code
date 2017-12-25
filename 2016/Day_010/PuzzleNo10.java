import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

class Bot
{
    static Bot CHOSEN_ONE = null;

    private List<Integer> chips;
    private int id;

    Bot(int id)
    {
        this.chips = new ArrayList<>();
        this.id = id;
    }

    int getChipValuesSum()
    {
        return chips.stream().mapToInt(v -> v).sum();
    }

    void addChip(int v)
    {
        chips.add(v);
        System.out.println(String.format("Bot %d receives chip %d", getId(), v));
        if (canOperate() && chips.contains(61) && chips.contains(17)) {
            Bot.CHOSEN_ONE = this;
        }
    }

    void giveLowTo(Bot bot)
    {
        Integer low = chips.stream().min(Integer::compare).orElse(0);

        bot.addChip(low);
        chips.remove(low);

        System.out.println(String.format("Bot %d handles low to %d", getId(), bot.getId()));
    }

    void giveHighTo(Bot bot)
    {
        Integer high = chips.stream().max(Integer::compare).orElse(0);

        bot.addChip(high);
        chips.remove(high);

        System.out.println(String.format("Bot %d handles high to %d", getId(), bot.getId()));
    }

    boolean canOperate()
    {
        return chips.size() == 2;
    }

    int getId()
    {
        return id;
    }
}

interface Command
{
    boolean execute(List<Bot> bots);

    String GIVE_CMD_PATTERN = "value (\\d+) goes to bot (\\d+)";
    String TRANSFER_CMD_PATTER = "bot (\\d+) gives low to (\\w+) (\\d+) and high to (\\w+) (\\d+)";

    static Command createCommand(String raw)
    {
        //Give command
        Pattern pattern = Pattern.compile(GIVE_CMD_PATTERN);
        Matcher matcher = pattern.matcher(raw);

        if (matcher.matches()) {
            return new GiveChipCommand(Integer.valueOf(matcher.group(1)),
                    Integer.valueOf(matcher.group(2)));
        }

        //Transfer command
        pattern = Pattern.compile(TRANSFER_CMD_PATTER);
        matcher = pattern.matcher(raw);

        if (matcher.matches()) {
            int transferringBotId = Integer.valueOf(matcher.group(1));
            int lowReceiver = matcher.group(2).equals("output") ? -1 * (Integer.valueOf(matcher.group(3)) + 1)
                    : Integer.valueOf(matcher.group(3));
            int highReceiver = matcher.group(4).equals("output") ? -1 * (Integer.valueOf(matcher.group(5)) + 1)
                    : Integer.valueOf(matcher.group(5));

            return new TransferChipsCommand(transferringBotId, lowReceiver, highReceiver);
        }

        return null;
    }
}

class GiveChipCommand
        implements Command
{
    private int chipValue;
    private int botId;

    GiveChipCommand(int chipValue, int botId)
    {
        this.chipValue = chipValue;
        this.botId = botId;
    }

    @Override
    public boolean execute(List<Bot> bots)
    {
        Bot bot = bots.stream()
                .filter(b -> b.getId() == this.botId)
                .findFirst().orElse(null);

        if (bot == null) {
            bot = new Bot(botId);
            bots.add(bot);
        }

        bot.addChip(chipValue);
        return true;
    }
}

class TransferChipsCommand
        implements Command
{
    private int botId;
    private int lowChipDestBotId;
    private int highChipDestBotId;

    TransferChipsCommand(int botId, int lowChipDestBotId, int highChipDestBotId)
    {
        this.botId = botId;
        this.lowChipDestBotId = lowChipDestBotId;
        this.highChipDestBotId = highChipDestBotId;
    }

    @Override
    public boolean execute(List<Bot> bots)
    {
        Bot lowReceiver = bots.stream().filter(b -> b.getId() == this.lowChipDestBotId).findFirst().orElse(null);
        Bot highReceiver = bots.stream().filter(b -> b.getId() == this.highChipDestBotId).findFirst().orElse(null);
        Bot transferringBot = bots.stream().filter(b -> b.getId() == this.botId).findFirst().orElse(null);

        if (transferringBot == null) {
            System.out.println(String.format("Transferring bot %d is null.", this.botId));
            return false;
        }

        if (!transferringBot.canOperate()) {
            System.out.println("Bot cannot operate.");
            return false;
        }

        if (lowReceiver == null) {
            lowReceiver = new Bot(lowChipDestBotId);
            bots.add(lowReceiver);
        }

        if (highReceiver == null) {
            highReceiver = new Bot(highChipDestBotId);
            bots.add(highReceiver);
        }

        transferringBot.giveLowTo(lowReceiver);
        transferringBot.giveHighTo(highReceiver);

        return true;
    }
}

public class PuzzleNo10
{
    public static void main(String[] args)
            throws IOException
    {
        List<Command> commands = Files.readAllLines(Paths.get("puzzle_10.input"))
                .stream()
                .map(Command::createCommand)
                .collect(Collectors.toList());

        List<Bot> bots = new ArrayList<>();

        while (!commands.isEmpty()) {
            commands = commands.stream().filter(c -> !c.execute(bots)).collect(Collectors.toList());
        }

        System.out.println("=====");
        System.out.println("The Chosen One bot ID = " + (Bot.CHOSEN_ONE == null ? "NONE" : Bot.CHOSEN_ONE.getId()));

        System.out.println("Multiplication of outputs 0,1,2 = " +
                bots.stream()
                        .filter(b -> Arrays.asList(-1, -2, -3).contains(b.getId()))
                        .mapToInt(Bot::getChipValuesSum)
                        .reduce((s, i) -> s *= i).orElse(0));
    }
}
