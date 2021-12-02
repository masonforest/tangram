use jni::objects::{JMap, JObject, JString};
use jni::sys::jobject;
use jni::JNIEnv;
use memmap::Mmap;
use std::collections::BTreeMap;
use tangram_core::predict::BinaryClassificationPredictOutput;
use tangram_core::predict::PredictInputValue;
use tangram_core::predict::{PredictInput, PredictOutput};

static INTEGER_CLASS: &str = "java/lang/Integer";
static STRING_CLASS: &str = "java/lang/String";
static DOUBLE_CLASS: &str = "java/lang/Double";

#[no_mangle]
pub extern "system" fn Java_TangramModel_predict(
    env: JNIEnv,
    object: JObject,
    input: JObject,
) -> jobject {
    let path = get_path(&env, object);
    let file = std::fs::File::open(path).unwrap();
    let bytes = unsafe { Mmap::map(&file).unwrap() };
    let model = tangram_model::from_bytes(&bytes).unwrap();
    let model = tangram_core::predict::Model::from(model);
    let input = jobj_to_predict_input(&env, input);
    let output = tangram_core::predict::predict(&model, &[input], &Default::default());

    output_to_jobj(&env, &output[0]).into_inner()
}

fn output_to_jobj<'a>(env: &'a JNIEnv, predict_output: &PredictOutput) -> JObject<'a> {
    let (class_name, probability) = match predict_output {
        PredictOutput::BinaryClassification(BinaryClassificationPredictOutput {
            class_name,
            probability,
            ..
        }) => (class_name, probability),
        _ => panic!(),
    };
    let binary_predict_output_class = env.find_class("TangramBinaryPredictOutput").unwrap();
    let string_value = env.new_string(class_name).unwrap();
    env.new_object(
        binary_predict_output_class,
        "(Ljava/lang/String;F)V",
        &[string_value.into(), (*probability).into()],
    )
    .unwrap()
}

fn get_path(env: &JNIEnv, obj: JObject) -> String {
    jobj_to_string(
        &env,
        env.get_field(obj, "path", "Ljava/lang/String;")
            .unwrap()
            .l()
            .unwrap(),
    )
}

fn jobj_to_predict_input(env: &JNIEnv, obj: JObject) -> PredictInput {
    let mut map: BTreeMap<String, PredictInputValue> = BTreeMap::new();
    let jmap = JMap::from_env(&env, obj).unwrap();
    for (jmap_key, jmap_value) in jmap.iter().unwrap() {
        let key = jobj_to_string(&env, jmap_key);
        let value = jobj_to_predict_input_value(&env, jmap_value);
        map.insert(key, value);
    }
    PredictInput(map)
}

fn jobj_to_predict_input_value(env: &JNIEnv, obj: JObject) -> PredictInputValue {
    if env.is_instance_of(obj, STRING_CLASS).unwrap() {
        PredictInputValue::String(jobj_to_string(env, obj))
    } else if env.is_instance_of(obj, INTEGER_CLASS).unwrap() {
        PredictInputValue::Number(jobj_to_i32(env, obj).into())
    } else if env.is_instance_of(obj, DOUBLE_CLASS).unwrap() {
        PredictInputValue::Number(jobj_double_to_f64(env, obj).into())
    } else {
        panic!("Input values must either be Strings or Numbers");
    }
}

fn jobj_to_string(env: &JNIEnv, obj: JObject) -> String {
    env.get_string(JString::from(obj)).unwrap().into()
}

fn jobj_double_to_f64(env: &JNIEnv, obj: JObject) -> f64 {
    env.call_method(obj, "doubleValue", "()D", &[])
        .unwrap()
        .d()
        .unwrap()
}

fn jobj_to_i32(env: &JNIEnv, obj: JObject) -> i32 {
    env.call_method(obj, "intValue", "()I", &[])
        .unwrap()
        .i()
        .unwrap()
}
