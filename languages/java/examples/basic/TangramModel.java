import java.util.Map;

class TangramModel {
  public native TangramBinaryPredictOutput predict(Map<String, Object> input);
  String path;

  static {
    System.loadLibrary("tangram_jni");
  }
  
  private TangramModel(String path)
    {
        this.path = path;
    }


  static TangramModel fromPath(String path) {
    return new TangramModel(path);
  }  
}

class TangramBinaryPredictOutput {
  String className;
  float probability;

  public TangramBinaryPredictOutput(String className, float probability) {
    this.className = className;
    this.probability = probability;
  }

  public String toString(){
     return "TangramBinaryPredictOutput { \"className\": \"" + this.className + "\", \"probability\": " + this.probability +" }";  
  }
}
