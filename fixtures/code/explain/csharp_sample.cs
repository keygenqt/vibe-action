public class ObservableProperty<T>
{
    private T _value;
    public event Action<T> ValueChanged;

    public T Value
    {
        get => _value;
        set
        {
            if (!EqualityComparer<T>.Default.Equals(_value, value))
            {
                _value = value;
                ValueChanged?.Invoke(_value);
            }
        }
    }
}
