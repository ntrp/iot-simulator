#[cfg(test)]
mod test {
    use futures_util::stream::StreamExt;

    use super::super::*;

    #[test]
    fn it_should_generate_historical_data_and_live() {
        let sensor: &mut Sensor = &mut Sensor {
            id: Uuid::new_v4(),
            name: "sensorA".to_string(),
            metadata: Default::default(),
            sampling_rate: 1000,
            value_generator: Default::default(),
        };
        let s = sensor_emitter(
            String::from("path"),
            sensor,
            Utc::now() - Duration::seconds(3),
            Utc::now() + Duration::seconds(1),
        );

        tokio_test::block_on(async {
            let payloads: Vec<SensorPayload> = s.collect().await;
            assert_eq!(5, payloads.len())
        })
    }
}
