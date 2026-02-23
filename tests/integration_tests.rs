use rust_project_linkchecker::link_checker;
use std::fs;

#[tokio::test]
async fn basic_check_links_test() {
    let input_path = "test_data/test_file.txt";
    let output_path = "test_data/output.md";
    let result = link_checker::check_links(input_path.into(), 32, output_path.into()).await;
    match result {
        Ok(_) => {
            let expected_file_path = "test_data/expected_file_out.txt";
            let f1 = fs::read(expected_file_path).expect("Failed to read file 1");
            let f2 = fs::read(output_path).expect("Failed to read file 2");
            assert_eq!(f1, f2)
        }
        Err(e) => panic!("{}", e),
    }
}
