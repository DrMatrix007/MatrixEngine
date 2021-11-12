using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.ECS.Behaviors
{
    public class ECSException : Exception
    {
        public ECSException(string message) : base(message) { }
        }
}
