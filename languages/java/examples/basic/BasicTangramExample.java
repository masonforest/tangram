import java.util.HashMap;
import java.util.Map;

class BasicTangramExample {
    public static void main(String[] args) {
        Map<String, Object> map=new HashMap<String, Object>();
        map.put("age", 63.0);
        map.put("gender", "male");
        TangramModel tangramModel = TangramModel.fromPath("heart_disease.tangram");
        TangramBinaryPredictOutput output = tangramModel.predict(map);
        System.out.println(output.toString());
    }
}
