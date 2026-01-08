import java.util.List;
import java.util.Map;

public class Deep {
    // Raw type usage - should trigger violation
    private List items;
    private Map data;

    public void fail() {
        System.exit(1);
    }
}
