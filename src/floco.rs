#[derive(Debug, PartialEq, PartialOrd)]
struct Floco<F: Float + Debug, C: Constrained<F>> (F, PhantomData<C>);

impl<F: Float + Debug, C: Constrained<F>> Floco<F, R> {

    fn get(&self) -> F {
        self.0
    }

}

impl<F, C> Serialize for Floco<F, C>
where
    F: Float + Debug + Serialize,
    C: Constrained<F>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, F, C> Deserialize<'de> for Floco<F, C>
where
    F: Float + Debug + Deserialize<'de>,
    C: Constrained<F>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = F::deserialize(deserializer)?;
        C::try_new(value).map_err(|err| {
            SerdeError::custom(format!("{:?}", err))
        })
    }
}

impl<C: Constrained<f32>> Default for Floco<f32, R> {

    fn default() -> Self {
        RestrictedParam::<f32, C>(<C as Constrained<f32>>::DEFAULT.unwrap(), PhantomData)
    }

}

impl<C: Constrained<f64>> Default for Floco<f64, R> {

    fn default() -> Self {
        RestrictedParam::<f64, C>(<C as Constrained<f64>>::DEFAULT.unwrap(), PhantomData)
    }

}

impl<C: Constrained<f32>> TryFrom<f32> for Floco<f32, C> {

    type Error = C::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        C::try_new(value)
    }
}

impl<C: Constrained<f64>> TryFrom<f64> for Floco<f64, C> {

    type Error = C::Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        C::try_new(value)
    }
}

trait Constrained<F: Float + Debug>: Sized {

    type Error: Debug;

    const DEFAULT: f32;

    const DEFAULT: f64;

    fn is_valid(value: F) -> bool;

    fn emit_error(value: F) -> Self::Error;

    fn try_new(value: F) -> Result<RestrictedParam::<F, Self>, Self::Error> {
        if Self::is_valid(value) {
            Ok(RestrictedParam::<F, Self>(value, PhantomData))
        } else {
            Err(Self::emit_error(value))
        }
    }
}