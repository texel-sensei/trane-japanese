use trane::{
    course_builder::{CourseBuilder, LessonBuilder, ExerciseBuilder, AssetBuilder},
    data::{CourseManifest, ExerciseManifestBuilder, LessonManifestBuilder},
};

const COURSE_ID: &str = "texel::languages::japanese::A1::1";

const HIRAGANA: &[(&str, &str)] = &[
    ("あ", "a"),
    ("い", "i"),
    ("う", "u"),
    ("え", "e"),
    ("お", "o"),
    ("か", "ka"),
    ("き", "ki"),
    ("く", "ku"),
    ("け", "ke"),
    ("こ", "ko"),
    ("さ", "sa"),
    ("し", "shi"),
    ("す", "su"),
    ("せ", "se"),
    ("そ", "so"),
    ("た", "ta"),
    ("ち", "chi"),
    ("つ", "tsu"),
    ("て", "te"),
    ("と", "to"),
    ("な", "na"),
    ("に", "ni"),
    ("ぬ", "nu"),
    ("ね", "ne"),
    ("の", "no"),
    ("は", "ha"),
    ("ひ", "hi"),
    ("ふ", "fu"),
    ("へ", "he"),
    ("ほ", "ho"),
    ("ま", "ma"),
    ("み", "mi"),
    ("む", "mu"),
    ("め", "me"),
    ("も", "mo"),
    ("や", "ya"),
    ("ゆ", "yu"),
    ("よ", "yo"),
    ("ら", "ra"),
    ("り", "ri"),
    ("る", "ru"),
    ("れ", "re"),
    ("ろ", "ro"),
    ("わ", "wa"),
    ("を", "wo"),
    ("ん", "n"),
];

fn hiragana_lessons() -> LessonBuilder {
    let lesson_id = format!("{COURSE_ID}::hiragana");

    let exercises = HIRAGANA.iter().flat_map(|(hiragana, romanji)| {
        let read_id = format!("{lesson_id}::{hiragana}");
        let read_hiragana = ExerciseBuilder {
            directory_name: format!("{hiragana}"),
            manifest_closure: Box::new(move |mut m| m.id(read_id.clone()).clone()),
            asset_builders: vec![
                AssetBuilder { file_name: "front.md".to_owned(), contents: format!("Read {hiragana} out aloud.") },
                AssetBuilder { file_name: "back.md".to_owned(), contents: format!("{hiragana} is pronounced {romanji}") },
            ],
        };

        let write_id = format!("{lesson_id}::{romanji}");
        let write_hiragana = ExerciseBuilder {
            directory_name: format!("{romanji}"),
            manifest_closure: Box::new(move |mut m| m.id(write_id.clone()).clone()),
            asset_builders: vec![
                AssetBuilder { file_name: "front.md".to_owned(), contents: format!("Write the hiragana for {romanji}.") },
                AssetBuilder { file_name: "back.md".to_owned(), contents: format!("{romanji} is written as {hiragana}") },
            ],
        };

        [read_hiragana, write_hiragana]
    }).collect();

    LessonBuilder {
        directory_name: "hiragana".to_owned(),
        exercise_manifest_template: ExerciseManifestBuilder::default()
            .course_id(COURSE_ID.to_owned())
            .lesson_id(lesson_id.clone())
            .exercise_type(trane::data::ExerciseType::Declarative)
            .exercise_asset(trane::data::ExerciseAsset::FlashcardAsset {
                front_path: "front.md".into(),
                back_path: Some("back.md".into()),
            })
            .clone(),
        manifest_closure: Box::new(move |mut m| m.id(lesson_id.clone()).clone()),
        exercise_builders: exercises,
        asset_builders: vec![],
    }
}

pub fn jp_a1_1_course() -> CourseBuilder {
    CourseBuilder {
        directory_name: "A1-1".to_owned(),
        course_manifest: CourseManifest {
            id: COURSE_ID.into(),
            name: "Japanese A1-1".into(),
            dependencies: vec![],
            superseded: vec![],
            description: Some("First level of japanese, suitable for absolute beginners".into()),
            authors: None,
            metadata: None,
            course_material: None,
            course_instructions: None,
            generator_config: None,
        },
        lesson_manifest_template: LessonManifestBuilder::default()
            .course_id(COURSE_ID.to_owned())
            .clone(),
        lesson_builders: vec![hiragana_lessons()],
        asset_builders: vec![],
    }
}
