using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Behaviors
{
    public class BehaviorNotFoundException : ECSException
    {
        public BehaviorNotFoundException(Type type) : base($"The Behavior { type.Name } is not found")
        {
        }
    }
}
