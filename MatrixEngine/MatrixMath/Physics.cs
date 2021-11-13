using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Utils;

namespace MatrixEngine.MatrixMath
{
    public static class Physics
    {
        public static bool IsColliding(this Rect rect1, Rect rect2)
        {
            return rect1.X < rect2.max.X &&
                   rect1.max.X > rect2.X &&
                   rect1.Y < rect2.max.Y &&
                   rect1.max.Y > rect2.Y;
        }


        public static float GetCollisionFix(Rect dynamicStartRect, Rect dynamicEndRect, Rect staticRect, Direction dir)
        {
            var isColliding = dynamicEndRect.IsColliding(staticRect);

            switch (dir)
            {
                case Direction.X:
                    if (!isColliding &&
                        (!((staticRect.cY - dynamicStartRect.cY).Abs() * 2 <
                           staticRect.height + dynamicStartRect.height) ||
                         dynamicStartRect.cX < staticRect.cX == dynamicEndRect.cX < staticRect.cX)) return 0;
                    if (dynamicStartRect.cX < staticRect.cX)
                    {
                        return dynamicEndRect.max.X - staticRect.X;
                    }

                    return dynamicEndRect.X - staticRect.max.X;

                case Direction.Y:
                    if (!isColliding &&
                        (!((staticRect.cX - dynamicStartRect.cX).Abs() * 2 <
                           staticRect.width + dynamicStartRect.width) ||
                         dynamicStartRect.cY < staticRect.cY == dynamicEndRect.cY < staticRect.cY)) return 0;

                    if (dynamicStartRect.cY < staticRect.cY)
                    {
                        return dynamicEndRect.max.Y - staticRect.Y;
                    }

                    return dynamicEndRect.Y - staticRect.max.Y;


                default:
                    throw new ArgumentOutOfRangeException(nameof(dir), dir, null);
            }
        }
    }
}