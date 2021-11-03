using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS.Behaviors;

namespace MatrixEngine.ECS
{
    public class TestBehavior : Behavior
    {
        protected override void OnStart()
        {
            Console.WriteLine("EZ start");
        }

        protected override void OnUpdate()
        {
        }

        public override void Dispose()
        {
        }
    }
}