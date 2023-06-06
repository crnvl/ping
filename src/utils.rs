use snowflake::SnowflakeIdGenerator;

pub fn generate_snowflake() -> i64 {
    let mut id_gen = SnowflakeIdGenerator::new(1, 1);
    id_gen.real_time_generate()
}