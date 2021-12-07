using MatrixEngine.MatrixMath;
using SFML.System;

namespace MatrixEngine.Behaviors;
public class RectBehavior : Behavior
{

    public Rect Rect;

    public RectBehavior(Rect? r = null)
    {
        Rect = r.HasValue ? r.Value : new Rect(0, 0, 1, 1);
    }

    //public Rect GetRect() => new Rect(Transform.Position, Transform.Scale.Multiply(Size));
    public Vector2f Size
    {
        get => Rect.Size;
        set => Rect.Size = value;
    }

    public Vector2f Position
    {
        get => Rect.Position;
        set => Rect.Position = value;
    }


    public override void Dispose()
    {
    }

    protected override void OnStart()
    {
    }

    protected override void OnUpdate()
    {
    }
}
