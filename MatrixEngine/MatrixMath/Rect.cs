using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.System;

namespace MatrixEngine.MatrixMath
{
    public struct Rect
    {
        public float X, Y, width, height;

        public Vector2f max
        {
            get => new Vector2f(X + width, Y + height);
        }

        public Vector2f min
        {
            get => new Vector2f(X, Y);
        }

        public float cX { get => X + width * 0.5f; }
        public float cY { get => Y + height * 0.5f; }

        public Rect SetPos(Vector2f pos)
        {
            (X, Y) = (pos.X, pos.Y);
            return this;
        }

        public void SetSize(Vector2f size)
        {
            (width, height) = (size.X, size.Y);
        }

        public void SetAll(Vector2f pos, Vector2f size)
        {
            SetSize(size);
            SetPos(pos);
        }

        public Rect(float x = 0, float y = 0, float width = 10, float height = 10)
        {
            this.X = x;
            this.Y = y;
            this.width = width;
            this.height = height;
        }

        public Rect(Vector2f pos, Vector2f size)
        {
            this.X = pos.X;
            this.Y = pos.Y;
            this.width = size.X;
            this.height = size.Y;
        }

        public new string ToString()
        {
            return $"Rect(x:{X.ToString("0.0")}, y:{Y.ToString("0.0")}, width:{width.ToString("0.0")}, height:{height.ToString("0.0")})";
        }

        public Vector2f Position
        {
            get => new Vector2f(X, Y);
            set
            {
                X = value.X;
                Y = value.Y;
            }
        }

        public Vector2f center
        {
            get => new Vector2f(cX, cY);
        }

        public Vector2f Size
        {
            get => new Vector2f(width, height);
            set
            {
                width = value.X;
                height = value.Y;
            }
        }

        public bool IsInside(Vector2f pos)
        {
            return (pos.X >= this.X && pos.Y >= this.Y && pos.X <= this.max.X && pos.Y <= this.max.Y);
        }

        public Rect Copy()
        {
            return new Rect(Position, Size);
        }

        public IEnumerable<Line> ToLines()
        {
            yield return new Line(Position, Position + Size.OnlyWithX());
            yield return new Line(Position, Position + Size.OnlyWithY());
            yield return new Line(Position + Size.OnlyWithY(), max);
            yield return new Line(Position + Size.OnlyWithX(), max);
        }
    }
}