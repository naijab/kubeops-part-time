use ndarray::prelude::*;

pub fn distribute(server: i32, job: i32) -> Array2<i32> {
    // TODO: Implement distribute array
    return array![[0, 1], [2, 3]];
}

#[cfg(test)]
mod blob_test {
    use super::*;

    #[test]
    fn it_2_server_4_job_should_return_0_1_and_2_3() {
        let actual = distribute(2, 4);
        let expect = array![[0, 1], [2, 3]];
        assert_eq!(actual, expect);
    }
}
